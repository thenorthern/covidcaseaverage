import * as types from "./actionTypes";
import initialState from "./initialState";

export default function getCountries(state = initialState, action) {
  switch (action.type) {
    case types.GET_COUNTRIES_SUCCESS:
      return { ...state, countries: action.countries.sort() };
    case types.GET_COUNTRY_DATA_SUCCESS:
      return {
        ...state,
        infections_per_14_days: action.infections_per_14_days,
      };
    default:
      return state;
  }
}
