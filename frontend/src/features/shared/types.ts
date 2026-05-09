export type AsyncRun = (action: () => Promise<void>) => Promise<boolean>;
