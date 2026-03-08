export type NexApiResponse<T> = {
  code: boolean;
  data?: T;
  description?: string;
  error?: string[];
  count?: number;
};
