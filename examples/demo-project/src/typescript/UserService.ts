/**
 * User service with various code patterns
 * Demonstrates: complexity, dead code, code smells in TypeScript
 */

// TODO: Add proper error handling
// FIXME: This service needs refactoring for better separation of concerns

interface User {
  id: number;
  name: string;
  email: string;
  role: 'admin' | 'user' | 'guest';
  active: boolean;
}

interface ValidationResult {
  valid: boolean;
  errors: string[];
}

// Unused type - dead code
interface DeprecatedUser {
  username: string;
  password: string;
}

// Unused import
// import { Logger } from './logger';

export class UserService {
  private users: Map<number, User> = new Map();
  // Magic number
  private readonly MAX_USERS = 1000;
  private cacheEnabled: boolean = true;

  constructor() {
    // TODO: Load users from database
  }

  /**
   * Add a new user - high complexity
   */
  addUser(user: User): ValidationResult {
    const errors: string[] = [];

    // Deep nesting level 1
    if (user.id > 0) {
      // Deep nesting level 2
      if (user.name.length > 0) {
        // Deep nesting level 3
        if (user.email.includes('@')) {
          // Deep nesting level 4
          if (user.role !== 'guest') {
            // Deep nesting level 5
            if (user.active) {
              // Deep nesting level 6
              if (!this.users.has(user.id)) {
                // Deep nesting level 7
                if (this.users.size < this.MAX_USERS) {
                  this.users.set(user.id, user);
                  return { valid: true, errors: [] };
                } else {
                  errors.push('Maximum users reached');
                }
              } else {
                errors.push('User already exists');
              }
            } else {
              errors.push('User must be active');
            }
          } else {
            errors.push('Guest users not allowed');
          }
        } else {
          errors.push('Invalid email');
        }
      } else {
        errors.push('Name is required');
      }
    } else {
      errors.push('Invalid ID');
    }

    return { valid: false, errors };
  }

  /**
   * Validate user with multiple conditions - high complexity
   */
  validateUser(user: User): ValidationResult {
    const errors: string[] = [];

    if (user.id < 1) {
      errors.push('Invalid ID');
    } else if (user.id > 99999) {
      errors.push('ID too large');
    } else {
      if (user.name.length < 3) {
        errors.push('Name too short');
      } else if (user.name.length > 100) {
        errors.push('Name too long');
      } else {
        if (!user.email.includes('@')) {
          errors.push('Invalid email format');
        } else if (!user.email.includes('.')) {
          errors.push('Invalid email domain');
        } else {
          if (user.role === 'guest') {
            errors.push('Guest role not allowed');
          } else if (user.role === 'admin') {
            if (!user.email.endsWith('admin.com')) {
              errors.push('Admins must use admin.com email');
            }
          }
        }
      }
    }

    return {
      valid: errors.length === 0,
      errors
    };
  }

  /**
   * Process user data with multiple paths
   */
  processUserData(userId: number, action: string): any {
    const user = this.users.get(userId);

    if (!user) {
      return { error: 'User not found' };
    }

    if (action === 'activate') {
      if (user.role === 'admin') {
        return { status: 'activated', role: 'admin' };
      } else if (user.role === 'user') {
        return { status: 'activated', role: 'user' };
      } else {
        return { status: 'activated', role: 'guest' };
      }
    } else if (action === 'deactivate') {
      if (user.active) {
        return { status: 'deactivated' };
      } else {
        return { status: 'already inactive' };
      }
    } else if (action === 'delete') {
      if (user.role === 'admin') {
        return { error: 'Cannot delete admin' };
      } else {
        this.users.delete(userId);
        return { status: 'deleted' };
      }
    } else if (action === 'update') {
      return { status: 'ready for update' };
    } else {
      return { error: 'Unknown action' };
    }
  }

  /**
   * Unused method - dead code
   */
  private legacyAuthenticate(email: string, password: string): boolean {
    // This is no longer used
    return true;
  }

  /**
   * Another unused method
   */
  private deprecatedCacheKey(userId: number): string {
    return `user:${userId}`;
  }

  /**
   * Empty method - code smell
   */
  clearCache(): void {
    // TODO: Implement cache clearing
  }

  /**
   * Unreachable code
   */
  validateEmail(email: string): boolean {
    if (email.includes('@')) {
      return true;
    } else {
      return false;
    }

    // Unreachable
    console.log('This will never execute');
    return false;
  }

  /**
   * Unused variable
   */
  processBatch(users: User[]): void {
    const processed = 0;
    const failed = 0;
    const startTime: number = Date.now(); // Never used
    const batchSize = 100; // Never used

    // Processing logic
  }
}

// Duplicate code - Type 2 clone
function processAddition(items: number[]): number {
  const result = items.reduce((sum, item) => sum + item, 0);
  console.log(`Processing addition: ${items} = ${result}`);
  if (result > 1000) {
    console.log('Large result detected');
  }
  return result;
}

// Duplicate code - Type 2 clone
function processSubtraction(items: number[]): number {
  const result = items.reduce((diff, item) => diff - item, 0);
  console.log(`Processing subtraction: ${items} = ${result}`);
  if (result < 0) {
    console.log('Negative result detected');
  }
  return result;
}

// Commented out code
function legacyTransform(value: number): number {
  // Old implementation
  // const result = value * 2;
  // if (result > 100) {
  //   return result;
  // }
  // return 0;

  // New implementation
  return value * 3;
}

// Unused variable
const DEBUG_MODE = true;

// Another unused constant
const MAX_RETRIES = 3;
