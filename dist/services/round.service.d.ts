export declare class RoundService {
    createRound(startPrice: number, mode: number): Promise<any>;
    getRound(id: string): Promise<any>;
    resolveRound(id: string, finalPrice: number): Promise<any>;
    getActiveRound(): Promise<any>;
}
