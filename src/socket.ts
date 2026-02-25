import { Server } from 'socket.io';
import { prisma } from './lib/prisma';

export const initializeSocket = (io: Server) => {
  io.on('connection', (socket) => {
    socket.on('subscribe:round', async (roundId: string) => {
      const round = await prisma.round.findUnique({ where: { id: roundId } });
      if (round) {
        socket.join(`round:${roundId}`);
        socket.emit('round:update', round);
      }
    });

    socket.on('subscribe:user', async (userId: string) => {
      const user = await prisma.user.findUnique({ where: { id: userId } });
      if (user) {
        socket.join(`user:${userId}`);
      }
    });
  });
};
