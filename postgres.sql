PGDMP     %                	    y           test    13.4    13.4     �           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false            �           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false            �           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false            �           1262    16394    test    DATABASE     h   CREATE DATABASE test WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE = 'English_United States.1252';
    DROP DATABASE test;
                postgres    false            �            1259    16397    tasks    TABLE     O   CREATE TABLE public.tasks (
    id integer NOT NULL,
    name text NOT NULL
);
    DROP TABLE public.tasks;
       public         heap    postgres    false            �            1259    16395    tasks_id_seq    SEQUENCE     �   CREATE SEQUENCE public.tasks_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 #   DROP SEQUENCE public.tasks_id_seq;
       public          postgres    false    201            �           0    0    tasks_id_seq    SEQUENCE OWNED BY     =   ALTER SEQUENCE public.tasks_id_seq OWNED BY public.tasks.id;
          public          postgres    false    200            #           2604    16400    tasks id    DEFAULT     d   ALTER TABLE ONLY public.tasks ALTER COLUMN id SET DEFAULT nextval('public.tasks_id_seq'::regclass);
 7   ALTER TABLE public.tasks ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    200    201    201            �          0    16397    tasks 
   TABLE DATA           )   COPY public.tasks (id, name) FROM stdin;
    public          postgres    false    201   �	       �           0    0    tasks_id_seq    SEQUENCE SET     ;   SELECT pg_catalog.setval('public.tasks_id_seq', 1, false);
          public          postgres    false    200            %           2606    16405    tasks tasks_pk 
   CONSTRAINT     L   ALTER TABLE ONLY public.tasks
    ADD CONSTRAINT tasks_pk PRIMARY KEY (id);
 8   ALTER TABLE ONLY public.tasks DROP CONSTRAINT tasks_pk;
       public            postgres    false    201            �   #   x�3�,�/.I/J-V0�2ApL�LS�=... 8r�     