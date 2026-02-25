"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.authMiddleware = void 0;
const prisma_1 = require("../lib/prisma");
const authMiddleware = async (req, res, next) => {
    const token = req.headers.authorization?.split(' ')[1];
    if (!token) {
        return res.status(401).json({ error: 'Unauthorized' });
    }
    const user = await prisma_1.prisma.user.findUnique({ where: { token } });
    if (!user) {
        return res.status(401).json({ error: 'Invalid token' });
    }
    req.user = user;
    next();
};
exports.authMiddleware = authMiddleware;
