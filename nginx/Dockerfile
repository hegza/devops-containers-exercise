FROM docker.io/library/nginx:stable-alpine

# Install PHP-FPM 8.2
RUN apk add --no-cache php82 php82-fpm

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY htpasswd /etc/nginx/.htpasswd

COPY index.html /var/www/index.html

COPY shutdown.php /var/www/scripts/shutdown.php
COPY shutdown.sh /var/www/scripts/shutdown.sh
RUN chmod +x /var/www/scripts/shutdown.sh

# Configure PHP-FPM to listen on TCP port 9000
RUN sed -i 's|^listen = .*|listen = 9000|' /etc/php82/php-fpm.d/www.conf

CMD php-fpm82 --daemonize && nginx -g 'daemon off;'
