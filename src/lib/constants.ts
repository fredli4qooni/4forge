import { Database, Globe, Layers, Server } from "@lucide/svelte";
import type { ServiceItem, SiteItem, LogMessage } from "../types";

export const INITIAL_SERVICES: ServiceItem[] = [
  {
    id: "caddy",
    name: "Caddy Web Server",
    type: "Reverse Proxy & Auto-HTTPS",
    version: "v2.9.1",
    ports: "80, 443, 2019",
    status: "stopped",
    icon: Globe,
  },
  {
    id: "mariadb",
    name: "MariaDB Database",
    type: "Relational Database",
    version: "v11.4.3",
    ports: "3306",
    status: "stopped",
    icon: Database,
  },
  {
    id: "php",
    name: "PHP-FPM Manager",
    type: "FastCGI Process",
    version: "PHP 8.3.16 (NTS)",
    ports: "9000",
    status: "stopped",
    icon: Server,
  },
  {
    id: "node",
    name: "Node.js Runtime",
    type: "JavaScript / TypeScript",
    version: "v22.14.0 LTS",
    ports: "Isolated",
    status: "running",
    icon: Layers,
  },
];

export const INITIAL_SITES: SiteItem[] = [];

export const INITIAL_LOGS: LogMessage[] = [];

