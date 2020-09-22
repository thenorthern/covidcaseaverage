import { call, put, takeLatest } from "redux-saga/effects";
import * as actions from "../actions";
import { getApiDataFor } from "../api";

function* fetchCountries() {
  try {
    const result = yield call(getApiDataFor, `listcountries`);

    yield put(actions.getCountriesSuccess(result.countries));
  } catch (err) {
    yield put(actions.getCountriesError(err));
  }
}

function* fetchCountryData(action) {
  try {
    const result = yield call(
      getApiDataFor,
      `infectionsperday/${action.country}`,
    );

    yield put(actions.getCountryDataSuccess(result.infections_per_14_days));
  } catch (err) {
    yield put(actions.getCountryDataError(err));
  }
}

export default function* countriesSaga() {
  yield takeLatest(actions.getCountries, fetchCountries);
  yield takeLatest(actions.getCountryData, fetchCountryData);
}
