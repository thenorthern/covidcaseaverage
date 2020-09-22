import { spawn } from "redux-saga/effects";
import countriesSaga from "./countriesSaga";

export default function* rootSaga() {
  yield spawn(countriesSaga);
}
