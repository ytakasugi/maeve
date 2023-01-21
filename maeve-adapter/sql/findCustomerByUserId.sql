SELECT
    *
FROM
    CUSTOMER C
WHERE
    1 = 1
    AND USER_ID = $1
    AND EXISTS (
        SELECT
            1
        FROM
            USERS U
        WHERE
            1 = 1
            AND C.USER_ID = U.ID
    )