export type ProcessStep =
  | 'CheckApiKey'
  | 'EnterSubdomain'
  | 'SelectLocations'
  | 'SelectAppointmentType'
  | 'EnterDays'
  | 'CollectContext'
  | 'CollectAnalytics'
  | 'Complete';
