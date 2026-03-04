import { BaseHeaders, V20240412Headers, V2Headers } from './constants';

type NexApiResponse = {
  code: boolean;
  data: any | null;
  description: string[] | null;
  error: string[] | null;
  count: number | null;
};

type HttpMethod = 'GET' | 'PATCH' | 'POST' | 'DELETE';

const baseUrl = 'https://nexhealth.info/';

export const createApiClient = (getToken: () => string | null) => {
  return async function makeApiCall(
    relativeUrl: string,
    method: HttpMethod = 'GET',
    body?: any,
    useBeta?: boolean
  ): Promise<NexApiResponse> {
    const token = getToken();

    const headers = {
      ...(token ? { Authorization: token } : {}),
      ...(useBeta ? V20240412Headers : V2Headers),
      ...BaseHeaders,
    };

    const url = baseUrl + relativeUrl;

    const response = await fetch(url, {
      method,
      body: body ? JSON.stringify(body) : undefined,
      headers,
    });

    if (!response.ok) {
      const data = await response.json().catch(() => null);

      throw new Error(
        data?.error?.join(', ') ||
          `Api call failed with status ${response.status}`
      );
    }

    return response.json() as Promise<NexApiResponse>;
  };
};
