import sys
import subprocess
import os
import shutil

# This build script compiles a nannou project into a snippet

def main():
    target_folder = sys.argv[1]
    
    try:
        # Execute wasm-pack build --release
        subprocess.run(["wasm-pack", "build", "--release"], check=True)
        
        # Change directory to website
        os.chdir("website")
        
        # Execute npm install
        subprocess.run(["npm", "install"], check=True)
        
        # Execute npm run build
        subprocess.run(["npm", "run", "build"], check=True)
        
        # Generate snippet folder
        dist_folder = "dist"
        target_files = os.listdir(dist_folder)
        
        # Remove "dist/index.js"
        index_file_path = os.path.join(dist_folder, "index.js")
        if os.path.exists(index_file_path):
            os.remove(index_file_path)
        
        # Move all files from "dist" to target_folder
        for file_name in target_files:
            file_path = os.path.join(dist_folder, file_name)
            if os.path.isfile(file_path):
                shutil.move(file_path, target_folder)
        
        # Remove the "dist" directory
        shutil.rmtree(dist_folder, ignore_errors=True)
        
        print(f"Successfully moved files to {target_folder}")
    
    except subprocess.CalledProcessError as e:
        print(f"An error occurred while executing: {e.cmd}")
        print(f"Return code: {e.returncode}")
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    main()
