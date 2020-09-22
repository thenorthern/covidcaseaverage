import * as types from "./actionTypes";

export function getCountries() {
  return { type: types.GET_COUNTRIES };
}

export function getCountriesSuccess(countries) {
  return { type: types.GET_COUNTRIES_SUCCESS, countries: countries };
}

export function getCountriesError() {
  return { type: types.GET_COUNTRIES_ERROR };
}

export function getCountryData(country) {
  return { type: types.GET_COUNTRY_DATA, country };
}

export function getCountryDataSuccess(infections_per_14_days) {
  return { type: types.GET_COUNTRY_DATA_SUCCESS, infections_per_14_days };
}

export function getCountryDataError() {
  return { type: types.GET_COUNTRY_DATA_ERROR };
}
