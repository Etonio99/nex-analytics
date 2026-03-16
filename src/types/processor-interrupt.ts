import { NexLocation } from './api/locations';
import { DataConfirmation } from './data-confirmation';

export interface ProcessorInterrupt {
  type:
    | 'MISSING_API_KEY'
    | 'INVALID_API_KEY'
    | 'MISSING_SUBDOMAIN'
    | 'LOCATION_REQUIRED'
    | 'NO_LOCATIONS_FOUND'
    | 'MISSING_DAYS'
    | 'MISSING_APPOINTMENT_TYPE_NAME'
    | 'INTERNAL_ERROR'
    | 'NEEDS_CONFIRMATION'
    | 'PERMISSION_DENIED'
    | 'NOT_FOUND';
  resolutionData?: InterruptResolutionData;
}

export const interruptMessages: Record<ProcessorInterrupt['type'], string> = {
  MISSING_API_KEY: 'Api key is required',
  INVALID_API_KEY: 'Api key is invalid',
  MISSING_SUBDOMAIN: 'Subdomain is required',
  LOCATION_REQUIRED: 'At least one location must be selected',
  INTERNAL_ERROR: 'An unexpected error occurred',
  NO_LOCATIONS_FOUND: 'No locations found',
  MISSING_DAYS: 'Days is required',
  MISSING_APPOINTMENT_TYPE_NAME: 'Appointment type name is required',
  NEEDS_CONFIRMATION: '',
  PERMISSION_DENIED: 'You do not have permission to access this practice',
  NOT_FOUND: '',
};

type InterruptResolutionData =
  | { type: 'STRING'; payload: string }
  | { type: 'NUMBER'; payload: number }
  | { type: 'LOCATIONS'; payload: LocationResolutionData }
  | { type: 'CONFIRMATION'; payload: DataConfirmation }
  | { type: 'None'; payload: null };

type LocationResolutionData = {
  locations: NexLocation[];
  selected_location_ids?: number[];
};
