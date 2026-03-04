export type AppointmentSlots = {
  lid: number;
  pid: number;
  operatory_id: number;
  slots: AppointmentSlot[];
  next_available_date?: string;
};

type AppointmentSlot = {
  time: string;
  end_time: string;
  operatory_id: number;
  provider_id: number;
};
