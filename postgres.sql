PGDMP         7            	    y           test    13.4    13.4 
    ?           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false            ?           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false            ?           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false            ?           1262    16394    test    DATABASE     h   CREATE DATABASE test WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE = 'English_United States.1252';
    DROP DATABASE test;
                postgres    false            ?            1259    16406    _sqlx_migrations    TABLE       CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);
 $   DROP TABLE public._sqlx_migrations;
       public         heap    postgres    false            ?            1259    16397    tasks    TABLE     L   CREATE TABLE public.tasks (
    id uuid NOT NULL,
    name text NOT NULL
);
    DROP TABLE public.tasks;
       public         heap    postgres    false            ?          0    16406    _sqlx_migrations 
   TABLE DATA           q   COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
    public          postgres    false    201   ?	       ?          0    16397    tasks 
   TABLE DATA           )   COPY public.tasks (id, name) FROM stdin;
    public          postgres    false    200   
       *           2606    16414 &   _sqlx_migrations _sqlx_migrations_pkey 
   CONSTRAINT     i   ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);
 P   ALTER TABLE ONLY public._sqlx_migrations DROP CONSTRAINT _sqlx_migrations_pkey;
       public            postgres    false    201            (           2606    16418    tasks tasks_pk 
   CONSTRAINT     L   ALTER TABLE ONLY public.tasks
    ADD CONSTRAINT tasks_pk PRIMARY KEY (id);
 8   ALTER TABLE ONLY public.tasks DROP CONSTRAINT tasks_pk;
       public            postgres    false    200            ?      x?????? ? ?      ?   q   x?Eͻ1??WAb????^.?_?0??.??ݶ??靜?ԄB?[(1m???}?^?y?K??i??Z޾.?F]u?U??|y??m?g??ޯ?? U??//?x????i'1     