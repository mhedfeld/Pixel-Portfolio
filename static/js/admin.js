//Borrowed from https://codepen.io/Hyperplexed/pen/rNrJgrd
document.addEventListener('DOMContentLoaded', function() {
  fetchProjects();
});
const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

let interval = null;

document.querySelector("h2").onmouseover = event => {  
  let iteration = 0;
  
  clearInterval(interval);
  
  interval = setInterval(() => {
    event.target.innerText = event.target.innerText
      .split("")
      .map((letter, index) => {
        if(index < iteration) {
          return event.target.dataset.value[index];
        }
      
        return letters[Math.floor(Math.random() * 26)]
      })
      .join("");
    
    if(iteration >= event.target.dataset.value.length){ 
      clearInterval(interval);
    }
    
    iteration += 1 / 3;
  }, 30);
}
//Fetch Project Fn 
function fetchProjects() {
  fetch('/projects_data')
      .then(response => response.json())
      .then(projects => {
          const projectsList = document.getElementById('projectsList');
          projectsList.innerHTML = '';
          projects.forEach(project => {
              const projectElement = createProjectElement(project);
              projectsList.appendChild(projectElement);
          });
      })
      .catch(error => console.error('Error fetching projects:', error));
}

function createProjectElement(project) {
  const projectDiv = document.createElement('div');
  projectDiv.className = 'project-edit';
  projectDiv.innerHTML = `
      <h4>${project.title}</h4>
      <form class="edit-project-form" data-project-id="${project.id}">
          <input type="text" name="title" value="${project.title}" required>
          <textarea name="description" required>${project.description}</textarea>
          <input type="text" name="link" value="${project.link.replace('/project/', '')}" required>
          <input type="url" name="image" value="${project.image}" required>
          <input type="text" name="tech_stack" value="${project.tech_stack || ''}" placeholder="Tech Stack">
          <textarea name="challenges" placeholder="Challenges">${project.challenges || ''}</textarea>
          <textarea name="future_plans" placeholder="Future Plans">${project.future_plans || ''}</textarea>
          <button type="submit">Update Project</button>
          <button type="button" class="delete-project" data-project-id="${project.id}">Delete Project</button>
      </form>
  `;

  const form = projectDiv.querySelector('form');
  form.addEventListener('submit', function(e) {
      e.preventDefault();
      updateProject(project.id, new FormData(form));
  });

  const deleteButton = projectDiv.querySelector('.delete-project');
  deleteButton.addEventListener('click', function() {
      deleteProject(project.id);
  });

  return projectDiv;
}

//Update Project Fn 
function updateProject(projectId, formData) {
  const projectData = Object.fromEntries(formData.entries());
  
  // Handle the link field
  if (!projectData.link.startsWith('http://') && !projectData.link.startsWith('https://') && !projectData.link.startsWith('/project/')) {
      projectData.link = `/project/${projectData.link}`;
  }

  fetch(`/admin/update_project/${projectId}`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: JSON.stringify(projectData)
  })
  .then(response => response.json())
  .then(data => {
      if (data.success) {
          showNotification('Project updated successfully', 'success');
          fetchProjects();
      } else {
          showNotification('Failed to update project', 'error');
      }
  })
  .catch(error => {
      console.error('Error updating project:', error);
      showNotification('Error updating project', 'error');
  });
}

document.getElementById('addProjectForm').addEventListener('submit', function(e) {
  e.preventDefault();
  const formData = new FormData(this);
  const projectData = Object.fromEntries(formData.entries());
  
  // Handle the link field
  if (!projectData.link.startsWith('http://') && !projectData.link.startsWith('https://')) {
      projectData.link = `/project/${projectData.link}`;
  }

  fetch('/admin/add_project', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: JSON.stringify(projectData)
  })
  .then(response => response.json())
  .then(data => {
      if (data.success) {
          showNotification('Project added successfully', 'success');
          this.reset();
          fetchProjects();
      } else {
          showNotification('Failed to add project', 'error');
      }
  })
  .catch(error => {
      console.error('Error adding project:', error);
      showNotification('Error adding project', 'error');
  });
});

function deleteProject(projectId) {
  if (confirm('Are you sure you want to delete this project?')) {
      fetch(`/admin/delete_project/${projectId}`, {
          method: 'POST'
      })
      .then(response => response.json())
      .then(data => {
          if (data.success) {
              alert('Project deleted successfully');
              fetchProjects(); // Refresh the projects list
          } else {
              alert('Failed to delete project');
          }
      })
      .catch(error => console.error('Error deleting project:', error));
  }
}

//Notification Fn 
function showNotification(message, type) {
  const notification = document.createElement('div');
  notification.className = `notification ${type}`;
  notification.textContent = message;
  document.body.appendChild(notification);
  setTimeout(() => {
      notification.remove();
  }, 3000);
}