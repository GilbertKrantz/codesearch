/**
 * Pipeline module - circular dependency with DataFlow
 */

import { processData, validatePacket, transformPacket, DataPacket } from './DataFlow';

export interface PipelineResult {
  success: boolean;
  data?: any;
  errors?: string[];
}

/**
 * Process flow - circular call back to DataFlow
 */
export function processFlow(packet: DataPacket): boolean {
  console.log(`Processing flow for packet: ${packet.id}`);

  // Transform the packet
  const transformed = transformPacket(packet);

  // Circular call back to DataFlow
  if (validatePacket(transformed)) {
    return processData(transformed);
  } else {
    return false;
  }
}

/**
 * Execute pipeline with orchestration - creates circular flow
 */
export function executePipeline(packets: DataPacket[]): PipelineResult {
  const results: any[] = [];
  const errors: string[] = [];

  for (const packet of packets) {
    // Validate using DataFlow function
    if (!validatePacket(packet)) {
      errors.push(`Invalid packet: ${packet.id}`);
      continue;
    }

    // Process using DataFlow function (circular)
    const success = processData(packet);

    if (success) {
      // Circular call back to processFlow
      const flowSuccess = processFlow(packet);

      if (flowSuccess) {
        results.push(packet.id);
      } else {
        errors.push(`Flow failed for: ${packet.id}`);
      }
    } else {
      errors.push(`Processing failed for: ${packet.id}`);
    }
  }

  return {
    success: errors.length === 0,
    data: results,
    errors
  };
}

/**
 * Batch process with circular dependencies
 */
export function batchProcess(packets: DataPacket[]): boolean {
  let allSuccess = true;

  for (const packet of packets) {
    // Circular call chain
    const valid = validatePacket(packet);

    if (valid) {
      // Call to DataFlow
      const processed = processData(packet);

      if (processed) {
        // Circular back to processFlow
        const flowed = processFlow(packet);

        if (!flowed) {
          allSuccess = false;
        }
      } else {
        allSuccess = false;
      }
    } else {
      allSuccess = false;
    }
  }

  return allSuccess;
}
