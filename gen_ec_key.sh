openssl ecparam -name secp256k1 -genkey -noout -out private.ec.key
openssl pkcs8 -topk8 -in private.ec.key -out private.pem
openssl ec -in private.pem -pubout -out public.pem
