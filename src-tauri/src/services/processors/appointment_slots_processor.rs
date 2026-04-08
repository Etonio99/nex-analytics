use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use chrono::{Duration, Local, NaiveDate};
use rust_xlsxwriter::{workbook::Workbook, Format, Table, TableColumn, TableFunction, XlsxError};
use serde::{Deserialize, Serialize};
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
                InputData, InputField, ProcessorError, ProcessorInputRequest, ProcessorInterrupt,
                SelectData, SelectOption,
            },
        },
    },
    utils::{app_state::AppState, format_location_address},
};

pub struct AppointmentSlotsProcessor {
    pub app_state: Arc<AppState>,
    pub current_step: ProcessStep,
    pub target_step: Option<ProcessStep>,
    pub data: AppointmentSlotsProcessorData,
    pub file_path: Option<String>,
    pub api_call_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
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

    fn create_error(error: ProcessorError) -> ProcessorInterrupt {
        ProcessorInterrupt::Error(error)
    }

    fn create_input_request(
        title: &str,
        description: &str,
        input_field: InputField,
    ) -> ProcessorInterrupt {
        ProcessorInterrupt::InputRequired(ProcessorInputRequest {
            title: title.into(),
            description: description.into(),
            input_field,
        })
    }

    fn locations_to_select_options(locations: &[Location]) -> Vec<SelectOption> {
        locations
            .iter()
            .map(|l| SelectOption {
                title: l.name.clone(),
                subtitle: format_location_address(l),
                key: l.id,
            })
            .collect()
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
                    .map_err(|_| Self::create_error(ProcessorError::InternalError))?
                    .is_none()
                {
                    return Err(self.get_interrupt_for_current_step().await);
                }

                let response = client
                    .get_authenticates()
                    .await
                    .map_err(|_| Self::create_error(ProcessorError::InternalError))?;
                if !response.code {
                    return Err(Self::create_error(ProcessorError::InvalidApiKey));
                }

                self.current_step = ProcessStep::EnterSubdomain;
            }
            ProcessStep::EnterSubdomain => {
                let has_subdomain = {
                    let guard = self.app_state.data.lock().await;
                    guard.subdomain.is_some()
                };

                if !has_subdomain {
                    return Err(self.get_interrupt_for_current_step().await);
                }

                self.current_step = ProcessStep::FetchLocations;
            }
            ProcessStep::FetchLocations => {
                if self.data.locations.is_none() {
                    let guard = self.app_state.data.lock().await;

                    let Some(subdomain) = guard.subdomain.as_ref() else {
                        drop(guard);
                        self.current_step = ProcessStep::EnterSubdomain;
                        return Err(self.get_interrupt_for_current_step().await);
                    };

                    let locations_response = client
                        .get_locations(LocationsQuery {
                            subdomain: subdomain.clone(),
                            inactive: false,
                        })
                        .await
                        .map_err(|_| Self::create_error(ProcessorError::InternalError))?;

                    self.api_call_count += 1;
                    if !locations_response.code {
                        if let Some(e) = locations_response.error {
                            if e.contains(
                                &"You don't have access to perform this action.".to_string(),
                            ) {
                                return Err(Self::create_error(ProcessorError::PermissionDenied));
                            }
                        }
                    }

                    if let Some(institution_locations) = locations_response.data {
                        self.data.locations = Some(institution_locations[0].locations.clone());
                    } else {
                        return Err(Self::create_error(ProcessorError::NoLocationsFound));
                    }
                }

                self.current_step = ProcessStep::SelectLocations;
            }
            ProcessStep::SelectLocations => {
                let Some(_) = self.data.selected_location_ids else {
                    return Err(self.get_interrupt_for_current_step().await);
                };
                self.current_step = ProcessStep::EnterStartDate;
            }
            ProcessStep::EnterStartDate => {
                let Some(_) = self.data.start_date else {
                    return Err(self.get_interrupt_for_current_step().await);
                };
                self.current_step = ProcessStep::EnterDays;
            }
            ProcessStep::EnterDays => {
                let Some(_) = self.data.days else {
                    return Err(self.get_interrupt_for_current_step().await);
                };
                self.current_step = ProcessStep::EnterAppointmentTypeName;
            }
            ProcessStep::EnterAppointmentTypeName => {
                let Some(_) = self.data.appointment_type_name else {
                    return Err(self.get_interrupt_for_current_step().await);
                };
                self.current_step = ProcessStep::Confirmation;
            }
            ProcessStep::Confirmation => {
                if !self.data.confirmed.unwrap_or(false) {
                    return Err(self.get_interrupt_for_current_step().await);
                }
                self.current_step = ProcessStep::Processing;
            }
            ProcessStep::Processing => {
                self.process(client, app).await?;
                self.current_step = ProcessStep::Complete;
            }
            ProcessStep::Complete => {
                if !matches!(self.data.completion_acknowledged, Some(true)) {
                    if self.file_path.is_none() {
                        return Err(Self::create_error(ProcessorError::InternalError));
                    }
                    return Err(self.get_interrupt_for_current_step().await);
                }
            }
        }

        Ok(true)
    }

    async fn get_interrupt_for_current_step(&self) -> ProcessorInterrupt {
        match self.current_step {
            ProcessStep::CheckApiKey => Self::create_input_request(
                "API Key Required",
                "Please enter your Nex API key to continue.",
                InputField {
                    data: InputData::String(None),
                    label: Some("API Key".into()),
                    placeholder: Some("eCWxyomJxd56bv8.xPL7gwq...".into()),
                    description: None,
                    key: "api_key".into(),
                    required: true,
                },
            ),
            ProcessStep::EnterSubdomain => {
                let guard = self.app_state.data.lock().await;
                Self::create_input_request(
                    "Enter Subdomain",
                    "Please enter your institution's subdomain.",
                    InputField {
                        data: InputData::String(guard.subdomain.clone()),
                        label: Some("Subdomain".into()),
                        placeholder: Some("your-practice-subdomain".into()),
                        description: None,
                        key: "subdomain".into(),
                        required: true,
                    },
                )
            }
            ProcessStep::SelectLocations => {
                let locations = self.data.locations.clone().unwrap_or_default();
                Self::create_input_request(
                    "Select Locations",
                    "Select the locations you want to collect analytics from. Note that additional API calls will be made for each selected location.",
                    InputField {
                        data: InputData::Select(SelectData {
                            options: Self::locations_to_select_options(&locations),
                            selected_keys: self.data.selected_location_ids.clone(),
                        }),
                        label: Some("Locations".into()),
                        placeholder: None,
                        description: None,
                        key: "selected_location_ids".into(),
                        required: true,
                    },
                )
            }
            ProcessStep::EnterStartDate => Self::create_input_request(
                "Enter Start Date",
                "Enter the start date for the report timeframe.",
                InputField {
                    data: InputData::Date(self.data.start_date.clone()),
                    label: Some("Start Date".into()),
                    placeholder: Some("YYYY-MM-DD".into()),
                    description: None,
                    key: "start_date".into(),
                    required: true,
                },
            ),
            ProcessStep::EnterDays => Self::create_input_request(
                "Enter Days",
                "Enter the number of days to include in the report.",
                InputField {
                    data: InputData::Number(self.data.days),
                    label: Some("Days".into()),
                    placeholder: Some("e.g. 30".into()),
                    description: None,
                    key: "days".into(),
                    required: true,
                },
            ),
            ProcessStep::EnterAppointmentTypeName => Self::create_input_request(
                "Enter Appointment Type",
                "Enter the name of the appointment type to report on.",
                InputField {
                    data: InputData::String(self.data.appointment_type_name.clone()),
                    label: Some("Appointment Type Name".into()),
                    placeholder: Some("e.g. New Patient Cleaning".into()),
                    description: None,
                    key: "appointment_type_name".into(),
                    required: true,
                },
            ),
            ProcessStep::Confirmation => {
                let guard = self.app_state.data.lock().await;
                let confirmation_data = DataConfirmation {
                    subdomain: guard.subdomain.clone(),
                    locations_count: self
                        .data
                        .selected_location_ids
                        .as_ref()
                        .map(|v| v.len() as u32),
                    start_date: self.data.start_date.clone(),
                    days: self.data.days,
                    appointment_type_name: self.data.appointment_type_name.clone(),
                };
                Self::create_input_request(
                    "Confirm Report",
                    "Please confirm that all of the provided information is correct before proceeding.",
                    InputField {
                        data: InputData::Confirm(confirmation_data),
                        label: Some("Confirm".into()),
                        placeholder: None,
                        description: None,
                        key: "confirmed".into(),
                        required: true,
                    },
                )
            }
            ProcessStep::Complete => Self::create_input_request(
                "Report Complete",
                "Your report has been completed and was saved to the following location:",
                InputField {
                    data: InputData::AcknowledgeCompletion(self.file_path.clone()),
                    label: None,
                    placeholder: None,
                    description: None,
                    key: "completion_acknowledged".into(),
                    required: true,
                },
            ),
            _ => Self::create_error(ProcessorError::InternalError),
        }
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
                return Err(Self::create_error(ProcessorError::InternalError));
            };
            subdomain.clone()
        };

        let Some(appointment_type_name) = self.data.appointment_type_name.as_ref() else {
            return Err(Self::create_error(ProcessorError::InternalError));
        };

        let Some(start_date) = self.data.start_date.as_ref() else {
            return Err(Self::create_error(ProcessorError::InternalError));
        };

        let Some(days) = self.data.days.as_ref() else {
            return Err(Self::create_error(ProcessorError::InternalError));
        };

        let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|_| Self::create_error(ProcessorError::InternalError))?;

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
                    .map_err(|_| Self::create_error(ProcessorError::InternalError));

                let appointment_types_response = client
                    .get_appointment_types(AppointmentTypesQuery {
                        subdomain: subdomain.clone(),
                        location_id: *location_id,
                    })
                    .await
                    .map_err(|_| Self::create_error(ProcessorError::InternalError))?;
                self.api_call_count += 1;

                if !appointment_types_response.code {
                    println!("API call failed: {:#?}", appointment_types_response);
                    return Err(Self::create_error(ProcessorError::InternalError));
                }

                let Some(appointment_types) = appointment_types_response.data else {
                    return Err(Self::create_error(ProcessorError::InternalError));
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
                    .map_err(|_| Self::create_error(ProcessorError::InternalError))?;
                self.api_call_count += 1;

                if !providers_response.code {
                    println!("API call failed: {:#?}", providers_response);
                    return Err(Self::create_error(ProcessorError::InternalError));
                }

                let Some(providers_list) = providers_response.data else {
                    return Err(Self::create_error(ProcessorError::InternalError));
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
                    .map_err(|_| Self::create_error(ProcessorError::InternalError))?;
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
            .map_err(|_| Self::create_error(ProcessorError::InternalError))?
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
        .map_err(|_| Self::create_error(ProcessorError::InternalError))?;

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
        worksheet.set_column_width(0, 20)?;
        worksheet.set_column_width(1, 30)?;
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
            worksheet.set_column_width(0, 14)?;

            worksheet.write(0, 1, "Available Slots")?;
            worksheet.set_column_width(1, 14)?;

            let columns = vec![
                TableColumn::new()
                    .set_header("Date")
                    .set_total_label("Total"),
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
        let mut current = serde_json::to_value(&self.data).map_err(|e| e.to_string())?;

        if let (Some(current_obj), Some(update_obj)) = (current.as_object_mut(), data.as_object()) {
            for (key, value) in update_obj {
                if !value.is_null() {
                    current_obj.insert(key.clone(), value.clone());
                }
            }
        }

        self.data = serde_json::from_value(current)
            .map_err(|e| format!("Invalid data for Appointment Slots Processor: {}", e))?;

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
