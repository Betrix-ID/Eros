#!/system/bin/sh
 if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
	exit 1
 fi
 
#Chking cpu.abi
      if [ ! -f /sdcard/eros/sysctrl/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
		cp /sdcard/eros/sysctrl/target/release/arm64 /sdcard/eros/lnpng
    else 
         printf "Scripting ini tidak mendukung cpu.abi anda : $architecture \n"
      fi
   fi
#Smart notification
   shell() {
     cont="$1"
     cmd notification post -S bigtext -t '♨️ Multicor Priority ' 'Tag' "$cont" > /dev/null 2>&1 
   }
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "          ~ Description. Multicor Priority...... "
    echo
    echo "           - Author                  :  @UnixeID"
    echo "           - Point                     :  2.0 "
    echo "           - Release                :  25 - Mei - 2025"
    echo "           - Name Shell          :   Multicor Priority"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "   Priority Multicor Priority Custem. "
    sleep 2
    echo
        # Ambil daftar paket
      package_list=$(pm list package | cut -f2 -d: | tr -d '\r' | xargs -n1)
     control=1
       while IFS= read -r gamelist || [ -n "$gamelist" ]; do
          line=$(echo "$gamelist" | tr -d '\r' | xargs)
              if [ -n "$line" ]; then
        if echo "$package_list" | grep -xq "$line"; then
            echo "  $control. $line"
            control=$((control + 1))
          else
            echo "Paket game '$line' tidak ditemukan."
              fi
                fi
            done < /sdcard/eros/GamePid.txt
          # Buat varibale instlangi toast.apk
          if [ -f /sdcard/eros/toast.apk ]; then
    if pm list package | cut -f2 -d: | grep bellavita.toast; then
           echo
     else       
         cp /sdcard/eros/toast.apk /data/local/tmp
          pm install /data/local/tmp/toast.apk
       fi
     fi > /dev/null 2>&1       
     # Buat varibale instlling and uninstalling script
  if [ "$1" = kill ]; then
        if pgrep -f lnpng > /dev/null 2>&1; then
         echo "  Program is stopped in the backgurond "
         pm uninstall bellavita.toast > /dev/null 2>&1
         rm /data/local/tmp/* > /dev/null 2>&1
         pkill -f lnpng > /dev/null 2>&1
         shell "Program is stopped in the backgurond" 
         pkill -f sh > /dev/null 2>&1       
     else
       echo "Porgam faild stop !"
   fi
  else
     if ! pgrep -f lnpng > /dev/null 2>&1; then
       cp /sdcard/eros/lnpng /data/local/tmp
       chmod +x /data/local/tmp/lnpng
       nice -n -20 /data/local/tmp/lnpng > /dev/null 2>&1
    fi 
      sleep 2
        if pgrep -f lnpng > /dev/null 2>&1; then
        echo " Program is running in the backgurond"
     else
          echo "Porgram faild running !"
     fi
  fi 
set +x