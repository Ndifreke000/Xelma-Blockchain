"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.initializeSocket = void 0;
const prisma_1 = require("./lib/prisma");
const initializeSocket = (io) => {
    io.on('connection', (socket) => {
        socket.on('subscribe:round', async (roundId) => {
            const round = await prisma_1.prisma.round.findUnique({ where: { id: roundId } });
            if (round) {
                socket.join(`round:${roundId}`);
                socket.emit('round:update', round);
            }
        });
        socket.on('subscribe:user', async (userId) => {
            const user = await prisma_1.prisma.user.findUnique({ where: { id: userId } });
            if (user) {
                socket.join(`user:${userId}`);
            }
        });
    });
};
exports.initializeSocket = initializeSocket;
