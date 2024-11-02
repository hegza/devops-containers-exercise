FROM nginx:stable-alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf

# TODO: required?
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
