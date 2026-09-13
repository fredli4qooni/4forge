import type { Component } from "svelte";

export interface ServiceItem {
  id: string;
  name: string;
  type: string;
  version: string;
  ports: string;
  status: "stopped" | "starting" | "running" | "crashed";
  pid?: number;
  icon: Component<any>;
}

export interface SiteItem {
  domain: string;
  runtime: string;
  ssl: boolean;
  path: string;
  backend_type: string;
  target: string;
}

export interface LogMessage {
  service: string;
  stream: "stdout" | "stderr" | "system";
  message: string;
  timestamp_millis: number;
}

export interface PortCheckResult {
  port: number;
  is_available: boolean;
  service_name: string;
  process_id?: number;
  process_name?: string;
  alternative_port?: number;
}

export interface DetectedProject {
  name: string;
  path: string;
  domain: string;
  framework: string;
  runtime: string;
  web_root: string;
  backend_type: string;
  target: string;
}

export interface DatabaseLaunchResult {
  launched_type: string;
  client_name: string;
  url_or_path: string;
}

export interface UpdateCheck {
  current_version: string;
  channel: string;
  update_available: boolean;
  latest_version?: string;
  release_notes?: string;
  release_url?: string;
  signature_verified: boolean;
}

export type TabType = "dashboard" | "services" | "sites" | "runtimes" | "logs";

export type ViewMode = "compact" | "expanded";

export type DatabaseEngine = "mariadb" | "mysql" | "postgresql" | "sqlite";
