/**
 * Data flow module - part of circular dependency
 */

import { processFlow } from './Pipeline';

export interface DataPacket {
  id: string;
  payload: any;
  timestamp: number;
}

/**
 * Process data packet - circular call to Pipeline
 */
export function processData(packet: DataPacket): boolean {
  console.log(`Processing packet: ${packet.id}`);

  // Validate before processing
  if (packet.payload && packet.timestamp > 0) {
    // Circular call back to Pipeline
    return processFlow(packet);
  } else {
    return false;
  }
}

/**
 * Validate packet - called by Pipeline
 */
export function validatePacket(packet: DataPacket): boolean {
  return (
    packet.id.length > 0 &&
    packet.payload !== null &&
    packet.timestamp > 0
  );
}

/**
 * Transform packet
 */
export function transformPacket(packet: DataPacket): DataPacket {
  return {
    ...packet,
    timestamp: Date.now()
  };
}
