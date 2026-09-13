import { Database, Globe, Layers, Server, Zap } from "@lucide/svelte";
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
    name: "MySQL / MariaDB",
    type: "Relational Database",
    version: "v11.4.3",
    ports: "3306",
    status: "stopped",
    icon: Database,
  },
  {
    id: "postgresql",
    name: "PostgreSQL Database",
    type: "Relational Database",
    version: "v16.4",
    ports: "5432",
    status: "stopped",
    icon: Database,
  },
  {
    id: "redis",
    name: "Redis In-Memory Cache",
    type: "Key-Value & Queue Cache",
    version: "v7.2.5",
    ports: "6379",
    status: "stopped",
    icon: Zap,
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
  {
    id: "mongodb",
    name: "MongoDB Database",
    type: "NoSQL Document Store",
    version: "v7.0.14 Community",
    ports: "27017",
    status: "stopped",
    icon: Database,
  },
];

export const INITIAL_SITES: SiteItem[] = [];

export const INITIAL_LOGS: LogMessage[] = [];
