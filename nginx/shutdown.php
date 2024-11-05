<?php
echo "Shutting down the server...<br>";
$cmd = "/var/www/scripts/shutdown.sh";
echo "Calling $cmd...<br>";
$output = shell_exec('/var/www/scripts/shutdown.sh 2>&1');
echo "Command returned: $output<br>";
echo "<br><br>";
echo "As one can see, this approach couldn't quite pass the Linux security measures.<br><br>";
echo "I believe one appropriate approach would be to add shutdown functionality to all applications, <br>";
echo "then request that functionality from the nginx server. However, I've got a course to run so <br>";
echo "I gracefully give this one up. :)<br>";

exit;
?>