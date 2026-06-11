import { connect } from "@nats-io/transport-node";
import type { NatsConnection } from "@nats-io/nats-core";

export type NatsClientPair = {
  orgConn: NatsConnection;
  oseConn: NatsConnection;
};

const ORGANICLEVER_NATS_URL = process.env.ORGANICLEVER_NATS_URL ?? "nats://localhost:4222";
const OSE_NATS_URL = process.env.OSE_NATS_URL ?? "nats://localhost:4223";

let orgConn: NatsConnection | null = null;
let oseConn: NatsConnection | null = null;

export async function connectNats(): Promise<NatsClientPair> {
  orgConn = await connect({ servers: ORGANICLEVER_NATS_URL });
  oseConn = await connect({ servers: OSE_NATS_URL });
  // orgConn and oseConn are guaranteed non-null after the awaits above
  return { orgConn: orgConn!, oseConn: oseConn! };
}

export function getOrgConn(): NatsConnection {
  if (!orgConn) throw new Error("NATS orgConn not initialized; call connectNats() first.");
  return orgConn;
}

export function getOseConn(): NatsConnection {
  if (!oseConn) throw new Error("NATS oseConn not initialized; call connectNats() first.");
  return oseConn;
}

export async function drainNats(): Promise<void> {
  if (orgConn) {
    await orgConn.drain();
    orgConn = null;
  }
  if (oseConn) {
    await oseConn.drain();
    oseConn = null;
  }
}

export async function requestOnOrg(subject: string, data: Uint8Array): Promise<Uint8Array> {
  const conn = getOrgConn();
  const msg = await conn.request(subject, data, { timeout: 10000 });
  return msg.data;
}

export async function requestOnOse(subject: string, data: Uint8Array): Promise<Uint8Array> {
  const conn = getOseConn();
  const msg = await conn.request(subject, data, { timeout: 10000 });
  return msg.data;
}
