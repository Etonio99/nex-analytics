export type InstitutionLocations = {
  id: number;
  name: string;
  subdomain: string;
  locations: Location[];
};

export type Location = {
  id: number;
  name: string;
  institution_id: number;
  street_address?: string;
  street_address_2?: string;
  city?: string;
  state?: string;
  zip_code?: string;
  country_code?: string;
  created_at: string;
  updated_at: string;
  latitude?: number;
  longitude?: number;
  phone_number?: string;
  foreign_id?: string;
  foreign_id_type?: string;
  email?: string;
  tz: string;
  last_sync_time?: string;
  insert_appt_client: boolean;
  map_by_operatory: boolean;
  appt_types_map_by_operatory: boolean;
  set_availability_by_operatory: boolean;
  inactive: boolean;
  wlogo?: string;
  weight?: number;
};
