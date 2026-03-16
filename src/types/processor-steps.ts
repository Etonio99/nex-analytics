export type ProcessStep =
  | 'CheckApiKey'
  | 'EnterSubdomain'
  | 'FetchLocations'
  | 'SelectLocations'
  | 'EnterDays'
  | 'EnterAppointmentTypeName'
  | 'Confirmation'
  | 'Processing'
  | 'Complete';
