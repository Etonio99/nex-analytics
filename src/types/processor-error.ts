export interface ProcessorError {
  type:
    | 'MISSING_API_KEY'
    | 'INVALID_API_KEY'
    | 'MISSING_SUBDOMAIN'
    | 'LOCATION_REQUIRED'
    | 'INTERNAL_ERROR';
  details?: unknown;
}

export const errorMessages: Record<ProcessorError['type'], string> = {
  MISSING_API_KEY: 'Api key is required',
  INVALID_API_KEY: 'Api key is invalid',
  MISSING_SUBDOMAIN: 'Subdomain is required',
  LOCATION_REQUIRED: 'At least one location must be selected',
  INTERNAL_ERROR: 'An unexpected error occurred',
};
