export type ProcessStep =
  | 'CheckApiKey'
  | 'EnterSubdomain'
  | 'FetchLocations'
  | 'SelectLocations'
  | 'EnterStartDate'
  | 'EnterDays'
  | 'EnterAppointmentTypeName'
  | 'Confirmation'
  | 'Processing'
  | 'Complete';
