import React from "react";
import "./App.css";
import { useDispatch, useSelector } from "react-redux";
import { getCountries, getCountryData } from "./actions";
import Plot from "react-plotly.js";

const App = (props) => {
  const dispatch = useDispatch();
  const loadCountries = () => {
    dispatch(getCountries());
  };

  const getCountry = (event) => {
    if (event.target) {
      dispatch(getCountryData(event.target.value));
    }
  };

  loadCountries();
  const countries = useSelector((state) => state.countries.countries);
  const infections = useSelector(
    (state) => state.countries.infections_per_14_days,
  );

  const getX = (infections) => {
    return Array.from(Array(infections.length).keys());
  };

  const getY = (infections) => {
    return infections;
  };

  return (
    <div className="App">
      <select name="Countries" className="form-control" onChange={getCountry}>
        {countries.map((country) => {
          return (
            <option key={country} value={country}>
              {country}
            </option>
          );
        })}
      </select>
      <Plot
        data={[
          {
            x: getX(infections),
            y: getY(infections),
            type: "scatter",
            mode: "lines+markers",
            marker: { color: "red" },
          },
        ]}
        layout={{ width: 640, height: 480, title: "Cases per 100000 people" }}
      />
    </div>
  );
};

export default App;
