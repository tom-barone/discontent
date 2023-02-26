import "./menu.scss";
import "bootstrap";
import * as settings from "../settings";


(() => {
  // Add code to handle the button clicks
  // Use Stimulus https://stimulus.hotwired.dev/

  settings.get_icon("good").then((icon) => {
    (document.getElementById("settings-good") as HTMLInputElement).value = icon;
  });
  settings.get_icon("controversial").then((icon) => {
    (document.getElementById("settings-controversial") as HTMLInputElement).value = icon;
  });
  settings.get_icon("bad").then((icon) => {
    (document.getElementById("settings-bad") as HTMLInputElement).value = icon;
  });

})();
