export declare class SchedulerService {
    scheduleRoundResolution(roundId: string, resolveAt: Date): Promise<any>;
    getPendingTasks(): Promise<any>;
    completeTask(id: string): Promise<any>;
}
