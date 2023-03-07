// src/application.js
import { Application } from "@hotwired/stimulus";
import IconSettingController from "./controllers/icon_setting_controller";
import SettingsPageController from "./controllers/settings_page_controller";
import VotingButton from "./controllers/voting_button_controller";
import HelpButton from "./controllers/help_button_controller";
import "./popup.scss";
import "bootstrap";

window.Stimulus = Application.start();
window.Stimulus.register("icon-setting", IconSettingController);
window.Stimulus.register("settings-page", SettingsPageController);
window.Stimulus.register("voting-button", VotingButton);
window.Stimulus.register("help-button", HelpButton);
