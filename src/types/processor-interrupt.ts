// import { NexLocation } from './api/locations';
import { DataConfirmation } from './data-confirmation';

export type ProcessorInterrupt =
  | { type: 'ERROR'; payload: ProcessorError }
  | { type: 'INPUT_REQUIRED'; payload: InterruptPayload };

type ProcessorError =
  | { type: 'INVALID_API_KEY' }
  | { type: 'NO_LOCATIONS_FOUND' }
  | { type: 'PERMISSION_DENIED' }
  | { type: 'INTERNAL_ERROR' };

type InterruptPayload = {
  title: string;
  description: string;
  input_field: InputField;
};

type InputField = {
  label: string;
  placeholder?: string;
  description?: string;
  data: InputData;
  key: string;
};

type InputData =
  | { type: 'STRING'; payload: string }
  | { type: 'DATE'; payload: string }
  | { type: 'NUMBER'; payload: number }
  | { type: 'MULTI_STRING'; payload: MultiStringPayload }
  | { type: 'SELECT'; payload: SelectPayload }
  | { type: 'CONFIRM'; payload: DataConfirmation };

type MultiStringPayload = {
  options: string[];
  selected_strings?: string[];
};

type SelectPayload = {
  options: SelectOption[];
  selected_keys?: number[];
};

type SelectOption = {
  title: string;
  subtitle?: string;
  key: number;
};
