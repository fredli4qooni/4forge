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

export const INITIAL_SITES: SiteItem[] = [
  {
    domain: "laravel-app.test",
    runtime: "PHP 8.3",
    ssl: true,
    path: "C:\\projects\\laravel-app",
    backend_type: "fastcgi",
    target: "127.0.0.1:9000",
  },
  {
    domain: "dashboard-api.test",
    runtime: "Node 22",
    ssl: true,
    path: "C:\\projects\\dashboard-api",
    backend_type: "proxy",
    target: "127.0.0.1:3000",
  },
];

export const INITIAL_LOGS: LogMessage[] = [
  {
    service: "caddy",
    stream: "system",
    message: "Caddy reverse proxy initialized with internal local CA.",
    timestamp_millis: Date.now() - 60000,
  },
  {
    service: "mariadb",
    stream: "stdout",
    message: "mysqld.exe ready for connections on port 3306 (bind: 127.0.0.1).",
    timestamp_millis: Date.now() - 45000,
  },
  {
    service: "php",
    stream: "stdout",
    message: "php-cgi listening on 127.0.0.1:9000 with extensions (curl, pdo_mysql, mbstring, openssl).",
    timestamp_millis: Date.now() - 30000,
  },
];
