<?php
echo "Shutting down the server...<br>";
$output = shell_exec('/bin/sh /var/www/scripts/shutdown.sh 2>&1');
echo "output: $output<br>";
echo "Done?<br>";

exit;
?>