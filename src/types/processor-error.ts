import { NexLocation } from './api/locations';

export interface ProcessorError {
  type:
    | 'MISSING_API_KEY'
    | 'INVALID_API_KEY'
    | 'MISSING_SUBDOMAIN'
    | 'LOCATION_REQUIRED'
    | 'NO_LOCATIONS_FOUND'
    | 'MISSING_DAYS'
    | 'MISSING_APPOINTMENT_TYPE_NAME'
    | 'INTERNAL_ERROR';
  resolutionData?: ErrorResolutionData;
}

export const errorMessages: Record<ProcessorError['type'], string> = {
  MISSING_API_KEY: 'Api key is required',
  INVALID_API_KEY: 'Api key is invalid',
  MISSING_SUBDOMAIN: 'Subdomain is required',
  LOCATION_REQUIRED: 'At least one location must be selected',
  INTERNAL_ERROR: 'An unexpected error occurred',
  NO_LOCATIONS_FOUND: 'No locations found',
  MISSING_DAYS: 'Days is required',
  MISSING_APPOINTMENT_TYPE_NAME: 'Appointment type name is required',
};

type ErrorResolutionData =
  | { type: 'MESSAGE'; payload: string }
  | { type: 'LOCATIONS'; payload: NexLocation[] }
  | { type: 'None'; payload: null };
