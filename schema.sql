create table contracts
(
    id          text not null constraint contract_pk primary key,
    address     text,
    name        text,
    symbol      text,
    decimals    bigint
);

create table cursors
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);