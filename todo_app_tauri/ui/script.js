const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

// Общие константы и переменные
const importanceColor = {
    high: 'red',
    medium: 'orange',
    low: 'yellow',
    unmarked: 'grey'
}

const taskList = document.querySelector('.task-list');


// Отображение задачи
const taskCreation = function(taskId, taskName, taskLifesphere, taskImportance, taskState) {
    // Создание главных родителей-дивов
    let task = document.createElement('div');
    task.classList.add('task');
    task.id = "task-" + taskId;
    taskList.appendChild(task);

    let taskPartI = document.createElement('div');
    taskPartI.classList.add('task-part');
    taskPartI.classList.add('part-1');
    task.appendChild(taskPartI);

    let taskPartII = document.createElement('div');
    taskPartII.classList.add('task-part');
    taskPartII.classList.add('part-2');
    task.appendChild(taskPartII);

    // Наполнение дива part-1
    let taskCompletetionButton = document.createElement('button');
    taskCompletetionButton.classList.add("task-completion-square");
    taskPartI.appendChild(taskCompletetionButton);
    
    let taskImg = document.createElement('img');
    taskImg.width = "20";
    let taskImgSrc = 'img/square_';
    taskImgSrc += importanceColor[taskImportance];
    taskImgSrc += '_' + taskState + '.png';
    taskImg.src = taskImgSrc;
    taskCompletetionButton.appendChild(taskImg);

    let taskDesc = document.createElement('p');
    taskDesc.classList.add("task-desc");
    taskDesc.textContent = taskName;
    taskPartI.appendChild(taskDesc);

    // Наполнение дива part-2
    let taskLifesphereLabel = document.createElement('p');
    taskLifesphereLabel.classList.add("lifesphere-label");
    taskLifesphereLabel.classList.add("noselect");
    taskLifesphereLabel.textContent = taskLifesphere;
    taskPartII.appendChild(taskLifesphereLabel);

    let taskDeletionButton = document.createElement('button');
    taskDeletionButton.classList.add("task-deletion");
    taskPartII.appendChild(taskDeletionButton);
    
    let taskDeletionIcon = document.createElement('img');
    taskDeletionIcon.width = "27";
    taskDeletionIcon.src = "img/delete_white.png";
    taskDeletionButton.appendChild(taskDeletionIcon);
};

const displayTasks = function(tasksArray) {
    for (let i = 0; i < tasksArray.length; i++) {
        taskCreation(
            tasksArray[i].id, 
            tasksArray[i].name,
            tasksArray[i].lifesphere,
            tasksArray[i].importance.toLowerCase(),
            tasksArray[i].state.toLowerCase()
        );
    };
}

document.addEventListener("DOMContentLoaded", async function() {
    const addTaskLine = document.querySelector("form");
    let deleteTaskButtonElements;
    let completeTaskButtonElements;

    invoke('display_tasks_on_frontend').then((response) => { 
        displayTasks(response);
        setTimeout(function() {
            deleteTaskButtonElements = taskList.querySelectorAll(".task-deletion");
            deleteTaskButtonElements.forEach(function(button) {
                button.addEventListener("click", taskDeletion);
            }); 
            completeTaskButtonElements = taskList.querySelectorAll(".task-completion-square");
            completeTaskButtonElements.forEach(function(button) {
                button.addEventListener("click", taskCompletion);
                button.addEventListener("contextmenu", taskUnfinish);
            }); 
        }, 500);
    });
    
    const taskDeletion = function(evt) {
        let deletedTask = evt.target.parentNode.parentNode;
        let deletedTaskId = Number(deletedTask.id.slice(5));
        emit("delete-task-from-backend", deletedTaskId);
        location.reload();
    };

    const taskCompletion = function(evt) {
        let completedTask = evt.target.parentNode.parentNode;
        let completedTaskId = Number(completedTask.id.slice(5));
        emit("complete-task-in-backend", completedTaskId);
        location.reload();
    };

    const taskUnfinish = function(evt) {
        evt.preventDefault();
        let unfinishedTask = evt.target.parentNode.parentNode;
        let unfinishedTaskId = Number(unfinishedTask.id.slice(5));
        emit("unfinish-task-in-backend", unfinishedTaskId);
        location.reload();
    };

    
    // const addTask = function(value) {
    //     console.log(value);
    //     console.log(value);
    //     console.log(value);
    //     console.log(value);
    //     console.log(value);
    //     // emit("create-task-in-backend", value)
    // };
    addTaskLine.addEventListener("submit", (evt) => {
        const { elements } = evt.target;
        const data = Array.from(elements)
            .map((element) => {
            const { name, value } = element

            return { name, value }
        });

        let taskDesc = data[0].value;

        emit("add-task-to-backend", String(taskDesc))
    });
})



// // Контекстное меню
// document.oncontextmenu = rightClick;
  
// function rightClick(clickEvent) {
//     clickEvent.preventDefault();
//     // return false;
// }

// document.onclick = hideMenu;
// document.oncontextmenu = rightClick;
    
// function hideMenu() {
//     document.getElementById("contextMenu")
//             .style.display = "none"
// }

// function rightClick(e) {
//     e.preventDefault();

//     if (document.getElementById("contextMenu")
//             .style.display == "block")
//         hideMenu();
//     else{
//         var menu = document.getElementById("contextMenu")

//         menu.style.display = 'block';
//         menu.style.left = e.pageX + "px";
//         menu.style.top = e.pageY + "px";
//     }
// }
