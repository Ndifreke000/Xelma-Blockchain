import { prisma } from '../lib/prisma';

export class NotificationService {
  async createNotification(userId: string, message: string) {
    return prisma.notification.create({
      data: { userId, message, read: false }
    });
  }

  async getUserNotifications(userId: string) {
    return prisma.notification.findMany({
      where: { userId },
      orderBy: { createdAt: 'desc' }
    });
  }

  async markAsRead(id: string) {
    return prisma.notification.update({
      where: { id },
      data: { read: true }
    });
  }
}
