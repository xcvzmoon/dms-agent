import { TaggedError } from 'better-result';

export class TauriStoreError extends TaggedError('TauriStoreError')<{
  name: string;
  message: string;
  error: unknown;
}> {}

export class UploadError extends TaggedError('UploadError')<{
  message: string;
}> {}
