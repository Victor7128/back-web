CREATE TABLE IF NOT EXISTS item (
    id UUID PRIMARY KEY,
    nombre TEXT NOT NULL,
    descripcion TEXT NOT NULL,
    precio DECIMAL(10, 2) NOT NULL,
    cantidad INTEGER NOT NULL
);