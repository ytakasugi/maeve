-- Add migration script here
-- 商品テーブル
CREATE TABLE PRODUCT (
    ID SERIAL PRIMARY KEY,
    NAME TEXT NOT NULL,
    PRICE INTEGER NOT NULL,
    STOCK INTEGER NOT NULL,
    IMAGE_URL TEXT,
    DESCRIPTION TEXT
);

-- カテゴリテーブル
CREATE TABLE CATEGORY (
    ID SERIAL PRIMARY KEY,
    NAME TEXT NOT NULL,
    PARENT_ID INTEGER REFERENCES CATEGORY(ID)
);

-- ユーザーテーブル
CREATE TABLE USERS (
    ID VARCHAR(36) PRIMARY KEY,
    USER_NAME TEXT NOT NULL,
    EMAIL TEXT NOT NULL,
    PASSWORD_HASH TEXT NOT NULL,
    USER_ROLE TEXT NOT NULL
);

-- 顧客テーブル
CREATE TABLE CUSTOMER (
    ID VARCHAR(36) PRIMARY KEY,
    NAME TEXT NOT NULL,
    ZIP_CODE TEXT NOT NULL,
    ADDRESS TEXT NOT NULL,
    PHONE TEXT NOT NULL
);

-- 注文テーブル
CREATE TABLE ORDERS (
    ID SERIAL PRIMARY KEY,
    CUSTOMER_ID VARCHAR(36) NOT NULL REFERENCES CUSTOMER(ID),
    ORDER_DATETIME TIMESTAMPTZ NOT NULL,
    DELIVERY_ADDRESS TEXT NOT NULL,
    TOTAL_AMOUNT INTEGER NOT NULL,
    STATUS TEXT NOT NULL
);

-- 注文詳細テーブル
CREATE TABLE ORDER_ITEMS (
    ID SERIAL PRIMARY KEY,
    ORDER_ID INTEGER NOT NULL REFERENCES ORDERS(ID),
    PRODUCT_ID INTEGER NOT NULL REFERENCES PRODUCT(ID),
    QUANTITY INTEGER NOT NULL,
    UNIT_PRICE INTEGER NOT NULL
);

-- カートテーブル
CREATE TABLE CART (
    ID SERIAL PRIMARY KEY,
    CUSTOMER_ID VARCHAR(36) NOT NULL REFERENCES CUSTOMER(ID),
    PRODUCT_ID INTEGER NOT NULL REFERENCES PRODUCT(ID),
    QUANTITY INTEGER NOT NULL,
    UNIT_PRICE INTEGER NOT NULL
);

-- 配送先テーブル
CREATE TABLE DELIVERY_ADDRESS (
    ID SERIAL PRIMARY KEY,
    CUSTOMER_ID VARCHAR(36) NOT NULL REFERENCES CUSTOMER(ID),
    NAME TEXT NOT NULL,
    ADDRESS TEXT NOT NULL,
    PHONE TEXT NOT NULL
);

-- レビューテーブル
CREATE TABLE REVIEW (
    ID SERIAL PRIMARY KEY,
    CUSTOMER_ID VARCHAR(36) NOT NULL REFERENCES CUSTOMER(ID),
    PRODUCT_ID INTEGER NOT NULL REFERENCES PRODUCT(ID),
    RATING INTEGER NOT NULL,
    COMMENT TEXT
);

-- クーポンテーブル
CREATE TABLE COUPON (
    CODE TEXT PRIMARY KEY,
    EXPIRATION_DATE TIMESTAMPTZ NOT NULL,
    DISCOUNT_AMOUNT INTEGER,
    DISCOUNT_RATE INTEGER,
    USAGE_LIMIT INTEGER,
    USAGE_COUNT INTEGER,
    CONDITION TEXT
);

-- セッションテーブル
CREATE TABLE SESSION (
    ID TEXT PRIMARY KEY,
    DATA TEXT,
    EXPIRATION TIMESTAMPTZ
);

-- PWリセットテーブル
CREATE TABLE PASSWORD_RESET (
    EMAIL TEXT PRIMARY KEY,
    TOKEN TEXT NOT NULL,
    EXPIRATION TIMESTAMPTZ NOT NULL,
    USED BOOLEAN NOT NULL DEFAULT FALSE
);

-- アクセスログテーブル
CREATE TABLE ACCESS_LOG (
    ID SERIAL PRIMARY KEY,
    IP_ADDRESS TEXT NOT NULL,
    REQUEST_URL TEXT NOT NULL,
    REQUEST_METHOD TEXT NOT NULL,
    USER_AGENT TEXT NOT NULL,
    ACCESS_DATETIME TIMESTAMPTZ NOT NULL
);