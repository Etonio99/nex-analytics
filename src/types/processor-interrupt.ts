import { NexLocation } from './api/locations';
import { DataConfirmation } from './data-confirmation';

export interface ProcessorInterrupt {
  type:
    | 'MISSING_API_KEY'
    | 'INVALID_API_KEY'
    | 'MISSING_SUBDOMAIN'
    | 'LOCATION_REQUIRED'
    | 'NO_LOCATIONS_FOUND'
    | 'MISSING_START_DATE'
    | 'MISSING_DAYS'
    | 'MISSING_APPOINTMENT_TYPE_NAME'
    | 'INTERNAL_ERROR'
    | 'NEEDS_CONFIRMATION'
    | 'PERMISSION_DENIED'
    | 'NOT_FOUND';
  resolutionData?: InterruptResolutionData;
}

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
