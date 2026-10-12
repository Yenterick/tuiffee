-- This file should undo anything in `up.sql`

DELETE FROM method WHERE name IN ('V60-1', 'V60-2');

DELETE FROM variety WHERE name IN (
  'Arusha', 'Batian', 'Benguet', 'Bergendal', 'Bernardina', 'Blue Mountain', 
  'Bonifieur', 'Bourbon', 'Castillo', 'Catimor', 'Catuai', 'Caturra', 
  'Cenicafé 1', 'Charrier', 'Chiroso', 'Colombia', 'Ethiopian Heirloom', 
  'French Mission', 'Geisha', 'Harar', 'Java', 'K7', 'Kona', 'Laurina', 
  'Maracaturra', 'Maragogipe', 'Mayaguez', 'Mocha', 'Mundo Novo', 
  'Orange Bourbon', 'Pacamara', 'Pacas', 'Pink Bourbon', 'Ruiru 11', 
  'Sidamo', 'Sidikalang', 'SL28', 'SL34', 'Sudan Rume', 'Tabi', 'Typica', 
  'Villa Sarchi', 'Wush Wush', 'Yellow Bourbon', 'Yellow Caturra', 'Yirgacheffe'
);
