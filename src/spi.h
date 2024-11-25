#ifndef SPI_H
#define SPI_H

// Initialisation de SPI
void spi_init(void);

// Envoi de données via SPI
void spi_send(uint8_t data);

// Réception de données via SPI
uint8_t spi_receive(void);

#endif
