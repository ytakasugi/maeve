INSERT INTO CUSTOMER (ID, NAME, EMAIL, ADDRESS, PHONE)
VALUES ($1, $2, $3, $4, $5)
RETURNING *