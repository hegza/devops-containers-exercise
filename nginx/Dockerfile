FROM docker.io/library/nginx:stable-alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY htpasswd /etc/nginx/.htpasswd

CMD ["nginx", "-g", "daemon off;"]
