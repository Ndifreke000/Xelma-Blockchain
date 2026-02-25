export declare class NotificationService {
    createNotification(userId: string, message: string): Promise<any>;
    getUserNotifications(userId: string): Promise<any>;
    markAsRead(id: string): Promise<any>;
}
