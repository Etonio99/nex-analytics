use std::{collections::HashMap, fmt::format, fs, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use chrono::{Duration, Local, NaiveDate};
use rust_xlsxwriter::{
    workbook::Workbook, Color, Format, FormatAlign, FormatBorder, Table, TableColumn,
    TableFunction, XlsxError,
};
use serde::Deserialize;
use tauri::{Emitter, Manager};

use crate::{
    api::{
        types::{
            appointment_slots::AppointmentSlotsQuery,
            appointment_types::AppointmentTypesQuery,
            locations::{Location, LocationsQuery},
            operatories::Operatory,
            providers::{Provider, ProvidersQuery},
        },
        NexApiClient,
    },
    commands::keys::get_api_key,
    services::processors::{
        traits::Processor,
        types::{
            appointment_slots_data::{
                AvailableSlotsInTimeframe, LocationAvailableSlots, LocationAvailableSlotsError,
            },
            data_confirmation::DataConfirmation,
            process_steps::ProcessStep,
            processor_advance_result::ProcessorAdvanceResult,
            processor_interrupt::{
                InterruptResolutionData, LocationResolutionData, ProcessorInterrupt,
            },
        },
    },
    utils::app_state::AppState,
};

pub struct AppointmentSlotsProcessor {
    pub app_state: Arc<AppState>,
    pub current_step: ProcessStep,
    pub target_step: Option<ProcessStep>,
    pub data: AppointmentSlotsProcessorData,
    pub file_path: Option<String>,
    pub api_call_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct AppointmentSlotsProcessorData {
    pub confirmed: Option<bool>,
    pub locations: Option<Vec<Location>>,
    pub selected_location_ids: Option<Vec<u32>>,
    pub start_date: Option<String>,
    pub days: Option<u32>,
    pub appointment_type_name: Option<String>,
    pub operatories: Option<Vec<Operatory>>,
    pub providers: Option<Vec<Provider>>,
    pub completion_acknowledged: Option<bool>,
}

impl AppointmentSlotsProcessor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state,
            current_step: ProcessStep::CheckApiKey,
            target_step: None,
            data: AppointmentSlotsProcessorData {
                confirmed: Some(false),
                locations: None,
                selected_location_ids: None,
                start_date: None,
                days: None,
                appointment_type_name: None,
                operatories: None,
                providers: None,
                completion_acknowledged: Some(false),
            },
            file_path: None,
            api_call_count: 0,
        }
    }

    async fn step(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<bool, ProcessorInterrupt> {
        if Some(self.current_step.clone()) == self.target_step {
            self.target_step = None;
            return Err(self.get_interrupt_for_current_step().await);
        }

        match self.current_step {
            ProcessStep::CheckApiKey => {
                if get_api_key()
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?
                    .is_none()
                {
                    return Err(ProcessorInterrupt::MissingApiKey);
                }

                let response = client.get_authenticates().await.map_err(|e| {
                    ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                        e.to_string(),
                    ))
                })?;
                if !response.code {
                    return Err(ProcessorInterrupt::InvalidApiKey);
                }

                self.current_step = ProcessStep::EnterSubdomain;
            }
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                if guard.subdomain.is_none() {
                    return Err(ProcessorInterrupt::MissingSubdomain(None));
                }

                self.current_step = ProcessStep::FetchLocations;
            }
            ProcessStep::FetchLocations => {
                if self.data.locations.is_none() {
                    let guard = self.app_state.data.lock().await;

                    let Some(subdomain) = guard.subdomain.as_ref() else {
                        return Err(ProcessorInterrupt::MissingSubdomain(None));
                    };

                    let locations_response = client
                        .get_locations(LocationsQuery {
                            subdomain: subdomain.clone(),
                            inactive: false,
                        })
                        .await
                        .map_err(|e| {
                            ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                                e.to_string(),
                            ))
                        })?;

                    self.api_call_count += 1;
                    if !locations_response.code {
                        if let Some(e) = locations_response.error {
                            if e.contains(
                                &"You don't have access to perform this action.".to_string(),
                            ) {
                                let permission_type = &Some("subdomain".to_string());
                                return Err(ProcessorInterrupt::PermissionDenied(
                                    self.wrap_str(permission_type)
                                        .unwrap_or(InterruptResolutionData::None),
                                ));
                            }
                        }
                    }

                    if let Some(institution_locations) = locations_response.data {
                        self.data.locations = Some(institution_locations[0].locations.clone());
                    } else {
                        return Err(ProcessorInterrupt::NoLocationsFound);
                    }
                }

                self.current_step = ProcessStep::SelectLocations;
            }
            ProcessStep::SelectLocations => {
                let Some(_) = self.data.selected_location_ids else {
                    return Err(ProcessorInterrupt::LocationRequired(Some(
                        InterruptResolutionData::Locations(LocationResolutionData {
                            locations: self.data.locations.clone().unwrap_or_default(),
                            selected_location_ids: None,
                        }),
                    )));
                };
                self.current_step = ProcessStep::EnterStartDate;
            }
            ProcessStep::EnterStartDate => {
                let Some(_) = self.data.start_date else {
                    return Err(ProcessorInterrupt::MissingStartDate(None));
                };
                self.current_step = ProcessStep::EnterDays;
            }
            ProcessStep::EnterDays => {
                let Some(_) = self.data.days else {
                    return Err(ProcessorInterrupt::MissingDays(None));
                };
                self.current_step = ProcessStep::EnterAppointmentTypeName;
            }
            ProcessStep::EnterAppointmentTypeName => {
                let Some(_) = self.data.appointment_type_name else {
                    return Err(ProcessorInterrupt::MissingAppointmentTypeName(None));
                };
                self.current_step = ProcessStep::Confirmation;
            }
            ProcessStep::Confirmation => {
                if !self.data.confirmed.unwrap_or(false) {
                    let guard = self.app_state.data.lock().await;

                    let locations_count = self
                        .data
                        .selected_location_ids
                        .as_ref()
                        .map(|v| v.len() as u32);

                    return Err(ProcessorInterrupt::NeedsConfirmation(
                        InterruptResolutionData::Confirmation(DataConfirmation {
                            subdomain: guard.subdomain.clone(),
                            locations_count,
                            start_date: self.data.start_date.clone(),
                            days: self.data.days,
                            appointment_type_name: self.data.appointment_type_name.clone(),
                        }),
                    ));
                }
                self.current_step = ProcessStep::Processing;
            }
            ProcessStep::Processing => {
                self.process(client, app).await?;
                self.current_step = ProcessStep::Complete;
            }
            ProcessStep::Complete => {
                if !matches!(self.data.completion_acknowledged, Some(true)) {
                    let Some(file_path) = &self.file_path else {
                        return Err(ProcessorInterrupt::InternalError(
                            InterruptResolutionData::String("File path was None".into()),
                        ));
                    };

                    return Err(ProcessorInterrupt::AcknowledgeCompletion(
                        InterruptResolutionData::String(file_path.to_string()),
                    ));
                }
            }
            _ => return Ok(false),
        }

        Ok(true)
    }

    async fn get_interrupt_for_current_step(&self) -> ProcessorInterrupt {
        match self.current_step {
            ProcessStep::CheckApiKey => ProcessorInterrupt::MissingApiKey,
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;

                if guard.subdomain.is_none() {
                    return ProcessorInterrupt::MissingSubdomain(None);
                }

                ProcessorInterrupt::MissingSubdomain(self.wrap_str(&guard.subdomain))
            }
            ProcessStep::SelectLocations => ProcessorInterrupt::LocationRequired(Some(
                InterruptResolutionData::Locations(LocationResolutionData {
                    locations: self.data.locations.clone().unwrap_or_default(),
                    selected_location_ids: self.data.selected_location_ids.clone(),
                }),
            )),
            ProcessStep::EnterStartDate => {
                ProcessorInterrupt::MissingStartDate(self.wrap_str(&self.data.start_date))
            }
            ProcessStep::EnterDays => {
                ProcessorInterrupt::MissingDays(self.wrap_num(self.data.days))
            }
            ProcessStep::EnterAppointmentTypeName => {
                ProcessorInterrupt::MissingAppointmentTypeName(
                    self.wrap_str(&self.data.appointment_type_name),
                )
            }
            _ => ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                "Unknown step".into(),
            )),
        }
    }

    fn wrap_str(&self, value: &Option<String>) -> Option<InterruptResolutionData> {
        value
            .as_ref()
            .map(|s| InterruptResolutionData::String(s.clone()))
    }

    fn wrap_num(&self, value: Option<u32>) -> Option<InterruptResolutionData> {
        value.map(InterruptResolutionData::Number)
    }

    async fn process(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<(), ProcessorInterrupt> {
        let mut available_slot_data: Vec<LocationAvailableSlots> = vec![];

        let subdomain = {
            let guard = self.app_state.data.lock().await;
            let Some(subdomain) = guard.subdomain.as_ref() else {
                return Err(ProcessorInterrupt::MissingSubdomain(None));
            };
            subdomain.clone()
        };

        let Some(appointment_type_name) = self.data.appointment_type_name.as_ref() else {
            return Err(ProcessorInterrupt::InternalError(
                InterruptResolutionData::String("Appointment type name is missing".into()),
            ));
        };

        let Some(start_date) = self.data.start_date.as_ref() else {
            return Err(ProcessorInterrupt::InternalError(
                InterruptResolutionData::String("Start date is missing".into()),
            ));
        };

        let Some(days) = self.data.days.as_ref() else {
            return Err(ProcessorInterrupt::InternalError(
                InterruptResolutionData::String("Days is missing".into()),
            ));
        };

        let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").map_err(|e| {
            ProcessorInterrupt::InternalError(InterruptResolutionData::String(e.to_string()))
        })?;

        let mut counter = 0;
        if let Some(location_ids) = self.data.selected_location_ids.clone() {
            for location_id in &location_ids {
                counter += 1;
                let _ = app
                    .emit(
                        "progress",
                        format!(
                            "Processing location {} of {}...",
                            counter,
                            location_ids.len()
                        ),
                    )
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    });

                let appointment_types_response = client
                    .get_appointment_types(AppointmentTypesQuery {
                        subdomain: subdomain.clone(),
                        location_id: *location_id,
                    })
                    .await
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?;
                self.api_call_count += 1;

                if !appointment_types_response.code {
                    println!("API call failed: {:#?}", appointment_types_response);
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::None,
                    ));
                }

                let Some(appointment_types) = appointment_types_response.data else {
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::String("Appointment types value is None".into()),
                    ));
                };

                let matched_appointment_type = appointment_types
                    .iter()
                    .find(|at| at.name.to_lowercase() == appointment_type_name.to_lowercase());

                let Some(appointment_type) = matched_appointment_type else {
                    available_slot_data.push(LocationAvailableSlots {
                        location_id: *location_id,
                        error: Some(LocationAvailableSlotsError::AppointmentTypeNotFound),
                        available_slots: None,
                    });
                    continue;
                };

                let providers_response = client
                    .get_providers(ProvidersQuery {
                        subdomain: subdomain.clone(),
                        location_id: *location_id,
                        inactive: false,
                        requestable: true,
                        per_page: 300,
                    })
                    .await
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?;
                self.api_call_count += 1;

                if !providers_response.code {
                    println!("API call failed: {:#?}", providers_response);
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::None,
                    ));
                }

                let Some(providers_list) = providers_response.data else {
                    return Err(ProcessorInterrupt::InternalError(
                        InterruptResolutionData::String("Providers value is None".into()),
                    ));
                };

                let provider_ids = providers_list.iter().map(|p| p.id).collect();

                let appointment_slots_response = client
                    .get_appointment_slots(AppointmentSlotsQuery {
                        subdomain: subdomain.clone(),
                        start_date,
                        days: *days,
                        appointment_type_id: appointment_type.id,
                        location_id: *location_id,
                        provider_ids,
                    })
                    .await
                    .map_err(|e| {
                        ProcessorInterrupt::InternalError(InterruptResolutionData::String(
                            e.to_string(),
                        ))
                    })?;
                self.api_call_count += 1;

                if !appointment_slots_response.code {
                    println!("API call failed: {:#?}", appointment_slots_response);
                    let error_message = appointment_slots_response
                        .error
                        .as_ref()
                        .and_then(|e| Some(e.join(", ")))
                        .unwrap_or("Unknown Error".to_string());
                    available_slot_data.push(LocationAvailableSlots {
                        location_id: *location_id,
                        error: Some(LocationAvailableSlotsError::CallFailure { error_message }),
                        available_slots: None,
                    });
                    continue;
                }

                let Some(slot_data) = appointment_slots_response.data else {
                    println!("Slot data is None");
                    available_slot_data.push(LocationAvailableSlots {
                        location_id: *location_id,
                        error: Some(LocationAvailableSlotsError::NoSlotData),
                        available_slots: None,
                    });
                    continue;
                };

                let mut counts_by_date: HashMap<String, u32> = (0..*days)
                    .map(|offset| (start_date + Duration::days(offset as i64)).to_string())
                    .map(|date| (date, 0))
                    .collect();

                for data in &slot_data {
                    if let Some(slots) = &data.slots {
                        for slot in slots {
                            let date_string = slot.time.date_naive().to_string();
                            *counts_by_date.entry(date_string).or_insert(0) += 1;
                        }
                    }
                }

                let mut available_slots: Vec<AvailableSlotsInTimeframe> = counts_by_date
                    .into_iter()
                    .map(|(day, available_slots_count)| AvailableSlotsInTimeframe {
                        day,
                        available_slots_count,
                    })
                    .collect();
                available_slots.sort_by(|a, b| a.day.cmp(&b.day));

                available_slot_data.push(LocationAvailableSlots {
                    location_id: *location_id,
                    error: None,
                    available_slots: Some(available_slots),
                });
            }
        }

        let save_path = app
            .path()
            .document_dir()
            .map_err(|e| {
                ProcessorInterrupt::InternalError(InterruptResolutionData::String(e.to_string()))
            })?
            .join("Nex Analytics")
            .join("Available Slots");

        self.write_workbook(
            save_path,
            start_date,
            *days,
            &subdomain,
            &available_slot_data,
        )
        .await
        .map_err(|e| {
            ProcessorInterrupt::InternalError(InterruptResolutionData::String(e.to_string()))
        })?;

        Ok(())
    }

    async fn write_workbook(
        &mut self,
        dir: PathBuf,
        start_date: chrono::NaiveDate,
        days: u32,
        subdomain: &String,
        data: &[LocationAvailableSlots],
    ) -> Result<(), XlsxError> {
        fs::create_dir_all(&dir)?;

        let format_bold = Format::new().set_bold();

        let now = Local::now();

        let file_name = format!(
            "available_slots_{}_{}d_{}.xlsx",
            start_date,
            days,
            now.format("%Y-%m-%dT%H-%M-%S%z")
        );
        let file_path = dir.join(file_name);

        self.file_path = file_path.to_str().map(|s| s.to_string());

        let mut workbook = Workbook::new();

        let worksheet = workbook.add_worksheet();
        worksheet.set_name("Summary")?;
        worksheet.write_with_format(0, 0, "Summary", &format_bold)?;
        worksheet.write(1, 0, "Processor")?;
        worksheet.write(1, 1, "Available Slots")?;
        worksheet.write(2, 0, "Start Date")?;
        worksheet.write(2, 1, start_date.to_string())?;
        worksheet.write(3, 0, "Days")?;
        worksheet.write(3, 1, days.to_string())?;
        worksheet.write(4, 0, "Appointment Type Name")?;
        worksheet.write(
            4,
            1,
            &self
                .data
                .appointment_type_name
                .clone()
                .unwrap_or("Failed to get appointment type name".to_string()),
        )?;
        worksheet.write(5, 0, "Subdomain")?;
        worksheet.write(5, 1, subdomain)?;
        worksheet.write(6, 0, "API Calls")?;
        worksheet.write(6, 1, self.api_call_count)?;

        for location_slot_data in data {
            let worksheet = workbook.add_worksheet();
            let location_name = self
                .data
                .locations
                .as_ref()
                .and_then(|locs| locs.iter().find(|l| l.id == location_slot_data.location_id))
                .map(|l| l.name.as_str())
                .unwrap_or("Unnamed Location");
            let full_name = format!("{} - {}", location_slot_data.location_id, location_name);
            let worksheet_name: String = full_name.chars().take(31).collect(); // Excel worksheet names are limited to 31 characters
            worksheet.set_name(worksheet_name)?;

            if let Some(error) = &location_slot_data.error {
                let error_msg = match error {
                    LocationAvailableSlotsError::AppointmentTypeNotFound => {
                        "Failed to find appointment type with the provided name".to_string()
                    }
                    LocationAvailableSlotsError::NoSlotData => {
                        "Appointment slots data was empty".to_string()
                    }
                    LocationAvailableSlotsError::CallFailure { error_message } => {
                        format!("Api call failed: {}", error_message)
                    }
                };
                worksheet.write(0, 0, error_msg.as_str())?;
                continue;
            }

            worksheet.write(0, 0, "Date")?;
            worksheet.set_column_width(0, 16)?;

            worksheet.write(0, 1, "Available Slots")?;
            worksheet.set_column_width(1, 22)?;

            let columns = vec![
                TableColumn::new()
                    .set_header("Date")
                    .set_total_label("Totals"),
                TableColumn::new()
                    .set_header("Available Slots")
                    .set_total_function(TableFunction::Sum),
            ];
            let table = Table::new().set_columns(&columns).set_total_row(true);

            worksheet.add_table(0, 0, *&days + 1, 1, &table)?;

            worksheet.set_freeze_panes(1, 1)?;

            if let Some(slots) = &location_slot_data.available_slots {
                for (i, entry) in slots.iter().enumerate() {
                    let row = (i + 1) as u32;
                    worksheet.write(row, 0, &entry.day)?;
                    worksheet.write(row, 1, entry.available_slots_count)?;
                }
            }
        }

        println!("Saving file to {:?}", file_path.to_str());
        workbook.save(file_path)?;

        Ok(())
    }
}

#[async_trait]
impl Processor for AppointmentSlotsProcessor {
    async fn advance(
        &mut self,
        client: &NexApiClient,
        app: &tauri::AppHandle,
    ) -> Result<ProcessorAdvanceResult, String> {
        let mut interrupt = None;

        loop {
            match self.step(client, app).await {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    interrupt = Some(e);
                    break;
                }
            }
        }

        Ok(ProcessorAdvanceResult {
            step: self.current_step.clone(),
            interrupt,
        })
    }

    fn update_data(&mut self, data: serde_json::Value) -> Result<(), String> {
        let input: AppointmentSlotsProcessorData = serde_json::from_value(data)
            .map_err(|e| format!("Invalid data for Appointment Slots Processor: {}", e))?;

        if let Some(c) = input.confirmed {
            self.data.confirmed = Some(c);
        }
        if let Some(l) = input.locations {
            self.data.locations = Some(l);
        }
        if let Some(l) = input.selected_location_ids {
            self.data.selected_location_ids = Some(l);
        }
        if let Some(s) = input.start_date {
            self.data.start_date = Some(s);
        }
        if let Some(d) = input.days {
            self.data.days = Some(d);
        }
        if let Some(a) = input.appointment_type_name {
            self.data.appointment_type_name = Some(a);
        }
        if let Some(o) = input.operatories {
            self.data.operatories = Some(o);
        }
        if let Some(p) = input.providers {
            self.data.providers = Some(p);
        }

        Ok(())
    }

    fn make_stale(&mut self) {
        self.data.locations = None;
        self.data.selected_location_ids = None;
    }

    fn jump_to_step(&mut self, step: ProcessStep) {
        self.current_step = step.clone();
        self.target_step = Some(step.clone());
    }
}
