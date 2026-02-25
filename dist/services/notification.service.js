"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.NotificationService = void 0;
const prisma_1 = require("../lib/prisma");
class NotificationService {
    async createNotification(userId, message) {
        return prisma_1.prisma.notification.create({
            data: { userId, message, read: false }
        });
    }
    async getUserNotifications(userId) {
        return prisma_1.prisma.notification.findMany({
            where: { userId },
            orderBy: { createdAt: 'desc' }
        });
    }
    async markAsRead(id) {
        return prisma_1.prisma.notification.update({
            where: { id },
            data: { read: true }
        });
    }
}
exports.NotificationService = NotificationService;
